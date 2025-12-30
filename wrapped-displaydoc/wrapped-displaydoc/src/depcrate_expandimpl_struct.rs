// Generated macro for impl_struct (function)
macro_rules! Depcrate_expandimpl_struct {
() => {
// Module: crate::expand
// Provides: {"impl_struct"}
// Dependencies: {}
fn impl_struct (input : & DeriveInput , data : & DataStruct) -> Result < TokenStream > { let ty = & input . ident ; let (impl_generics , ty_generics , where_clause) = input . generics . split_for_impl () ; let where_clause = generate_where_clause (& input . generics , where_clause) ; let helper = AttrsHelper :: new (& input . attrs) ; let display = helper . display (& input . attrs) ? . map (| display | { let pat = match & data . fields { Fields :: Named (fields) => { let var = fields . named . iter () . map (| field | & field . ident) ; quote ! (Self { # (# var) ,* }) } Fields :: Unnamed (fields) => { let var = (0 .. fields . unnamed . len ()) . map (| i | format_ident ! ("_{}" , i)) ; quote ! (Self (# (# var) ,*)) } Fields :: Unit => quote ! (_) , } ; quote ! { impl # impl_generics :: core :: fmt :: Display for # ty # ty_generics # where_clause { fn fmt (& self , formatter : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { # [allow (unused_variables)] let # pat = self ; # display } } } }) ; Ok (quote ! { # display }) }
};
}
