macro_rules! deps {
    () => {
        DemangleContext!();
        DemangleWrite!();
        LeafName!();
        ArgScopeStack!();
        Result!();
        GetLeafName!();
        SubstitutionTable!();
        Demangle!();
        Substitutable!();
    };
}

macro_rules! define_handle {
    () => {
        deps!();
        # [doc = " Define a handle to a AST type that lives inside the substitution table. A"] # [doc = " handle is always either an index into the substitution table, or it is a"] # [doc = " reference to a \"well-known\" component."] # [doc = ""] # [doc = " This declares:"] # [doc = ""] # [doc = " - The enum of either a back reference into the substitution table or a"] # [doc = "   reference to a \"well-known\" component"] # [doc = " - a `Demangle` impl that proxies to the appropriate `Substitutable` in the"] # [doc = "   `SubstitutionTable`"] macro_rules ! define_handle { ($ (# [$ attr : meta]) * pub enum $ typename : ident) => { define_handle ! { $ (# [$ attr]) * pub enum $ typename { } } } ; ($ (# [$ attr : meta]) * pub enum $ typename : ident { $ ($ (# [$ extra_attr : meta]) * extra $ extra_variant : ident ($ extra_variant_ty : ty) ,) * }) => { $ (# [$ attr]) * # [derive (Clone , Debug , PartialEq , Eq)] pub enum $ typename { # [doc = " A reference to a \"well-known\" component."] WellKnown (WellKnownComponent) , # [doc = " A back-reference into the substitution table to a component we"] # [doc = " have already parsed."] BackReference (usize) , $ ($ (# [$ extra_attr]) * $ extra_variant ($ extra_variant_ty) ,) * } impl $ typename { # [doc = " If this is a `BackReference`, get its index."] pub fn back_reference (& self) -> Option < usize > { match * self { $ typename :: BackReference (n) => Some (n) , _ => None , } } } impl <'subs , W > Demangle <'subs , W > for $ typename where W : 'subs + DemangleWrite { # [inline] fn demangle <'prev , 'ctx > (&'subs self , ctx : &'ctx mut DemangleContext <'subs , W >, scope : Option < ArgScopeStack <'prev , 'subs >>) -> fmt :: Result { match * self { $ typename :: WellKnown (ref comp) => comp . demangle (ctx , scope) , $ typename :: BackReference (idx) => ctx . subs [idx] . demangle (ctx , scope) , $ ($ typename ::$ extra_variant (ref extra) => extra . demangle (ctx , scope) ,) * } } } impl <'a > GetLeafName <'a > for $ typename { fn get_leaf_name (&'a self , subs : &'a SubstitutionTable) -> Option < LeafName <'a >> { match * self { $ typename :: WellKnown (ref wk) => wk . get_leaf_name (subs) , $ typename :: BackReference (idx) => { subs . get (idx) . and_then (| s | s . get_leaf_name (subs)) } $ ($ typename ::$ extra_variant (ref e) => e . get_leaf_name (subs) ,) * } } } } ; }
    };
}

define_handle!()