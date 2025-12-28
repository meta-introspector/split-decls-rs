macro_rules! deps {
    () => {
        FieldInfo!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a > FieldInfo < 'a > { pub fn make_list (iter : impl Iterator < Item = & 'a Field >) -> Vec < Self > { iter . enumerate () . map (| (i , field) | Self :: new_for_field (field , i)) . collect () } pub fn new_for_field (f : & 'a Field , index : usize) -> Self { if let Some (ref i) = f . ident { FieldInfo { accessor : quote ! (# i) , field : f , index , } } else { let idx = Index :: from (index) ; FieldInfo { accessor : quote ! (# idx) , field : f , index , } } } # [doc = " Get the code for setting this field in struct decl/brace syntax"] # [doc = ""] # [doc = " Use self.accessor for dot-notation accesses"] pub fn setter (& self) -> TokenStream2 { if let Some (ref i) = self . field . ident { quote ! (# i :) } else { quote ! () } } # [doc = " Produce a name for a getter for the field"] pub fn getter (& self) -> TokenStream2 { if let Some (ref i) = self . field . ident { quote ! (# i) } else { suffixed_ident ("field" , self . index , self . field . span ()) . into_token_stream () } } # [doc = " Produce a prose name for the field for use in docs"] pub fn getter_doc_name (& self) -> String { if let Some (ref i) = self . field . ident { format ! ("the unsized `{i}` field") } else { format ! ("tuple struct field #{}" , self . index) } } }
    };
}

impl_31!()