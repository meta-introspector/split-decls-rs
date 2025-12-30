// Generated macro for uses_lifetimes (macro)
macro_rules! Depcrate_macros_publicuses_lifetimes {
() => {
// Module: crate::macros_public
// Provides: {"uses_lifetimes"}
// Dependencies: {}
# [doc = " Generator for `UsesLifetimes` impls that unions the used lifetimes of the selected fields."] # [doc = ""] # [doc = " # Usage"] # [doc = " The macro takes the type implementing the trait as the first argument, then a comma-separated list of"] # [doc = " fields for the rest of its arguments."] # [doc = ""] # [doc = " The type of each passed-in field must implement `UsesLifetimes`, or the resulting code won't compile."] # [macro_export] macro_rules ! uses_lifetimes { ($ impl_type : ty , $ accessor : ident) => { impl $ crate :: usage :: UsesLifetimes for $ impl_type { fn uses_lifetimes <'gen > (& self , options : &$ crate :: usage :: Options , type_set : &'gen $ crate :: usage :: LifetimeSet) -> $ crate :: usage :: LifetimeRefSet <'gen > { self .$ accessor . uses_lifetimes (options , type_set) } } } ; ($ impl_type : ty , $ first : ident , $ ($ field : ident) ,+) => { impl $ crate :: usage :: UsesLifetimes for $ impl_type { fn uses_lifetimes <'gen > (& self , options : &$ crate :: usage :: Options , type_set : &'gen $ crate :: usage :: LifetimeSet) -> $ crate :: usage :: LifetimeRefSet <'gen > { let mut hits = self .$ first . uses_lifetimes (options , type_set) ; $ (hits . extend (self .$ field . uses_lifetimes (options , type_set)) ;) * hits } } } ; }
};
}
