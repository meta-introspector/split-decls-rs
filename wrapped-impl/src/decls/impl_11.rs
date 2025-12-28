macro_rules! deps {
    () => {
        MemberUnraw!();
        ParamsInScope!();
        IdentUnraw!();
        Field!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'a > Field < 'a > { fn multiple_from_syn (fields : & 'a Fields , scope : & ParamsInScope < 'a >) -> Result < Vec < Self > > { fields . iter () . enumerate () . map (| (i , field) | Field :: from_syn (i , field , scope)) . collect () } fn from_syn (i : usize , node : & 'a syn :: Field , scope : & ParamsInScope < 'a >) -> Result < Self > { Ok (Field { original : node , attrs : attr :: get (& node . attrs) ? , member : match & node . ident { Some (name) => MemberUnraw :: Named (IdentUnraw :: new (name . clone ())) , None => MemberUnraw :: Unnamed (Index { index : i as u32 , span : Span :: call_site () , }) , } , ty : & node . ty , contains_generic : scope . intersects (& node . ty) , }) } }
    };
}

impl_11!()