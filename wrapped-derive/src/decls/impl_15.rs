macro_rules! deps {
    () => {
        OwnULETy!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'a > OwnULETy < 'a > { fn new (ty : & 'a Type , context : & str) -> Result < Self , String > { match * ty { Type :: Slice (ref slice) => Ok (OwnULETy :: Slice (& slice . elem)) , Type :: Path (ref typath) => { if typath . path . is_ident ("str") { Ok (OwnULETy :: Str) } else { Err (format ! ("Cannot automatically detect corresponding VarULE type for non-str path type inside a {context}")) } } _ => Err (format ! ("Cannot automatically detect corresponding VarULE type for non-slice/path type inside a {context}")) , } } # [doc = " Get the tokens for the corresponding VarULE type"] fn varule_ty (& self) -> TokenStream2 { match * self { OwnULETy :: Slice (s) => quote ! ([# s]) , OwnULETy :: Str => quote ! (str) , } } }
    };
}

impl_15!();