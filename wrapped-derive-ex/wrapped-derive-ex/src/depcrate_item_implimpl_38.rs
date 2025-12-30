// Generated macro for impl_38 (impl)
macro_rules! Depcrate_item_implimpl_38 {
() => {
// Module: crate::item_impl
// Provides: {"impl_38"}
// Dependencies: {}
impl Args { fn from_attr_args (attr : TokenStream , op : Op) -> Result < Args > { let args : ArgList = parse2 (attr) ? ; let mut make_binary = false ; let mut make_assign = false ; for item in & args . items { let target_op = Op :: from_ident (item) ? ; if target_op . op != op . op { bail ! (item . span () , "expected `{}` or `{}`" , Op :: new (op . op , OpForm :: Binary) , Op :: new (op . op , OpForm :: Assign)) ; } match target_op . form { OpForm :: Binary => make_binary = true , OpForm :: Assign => make_assign = true , } } Ok (Self { dump : args . dump , make_binary , make_assign , }) } }
};
}
