macro_rules! deps {
    () => {
        BuilderMethods!();
    };
}

macro_rules! MonoItemExt {
    () => {
        deps!();
        pub trait MonoItemExt < 'a , 'tcx > { fn define < Bx : BuilderMethods < 'a , 'tcx > > (& self , cx : & 'a mut Bx :: CodegenCx , cgu_name : & str , item_data : MonoItemData ,) ; fn predefine < Bx : BuilderMethods < 'a , 'tcx > > (& self , cx : & 'a mut Bx :: CodegenCx , cgu_name : & str , linkage : Linkage , visibility : Visibility ,) ; fn to_raw_string (& self) -> String ; }
    };
}

MonoItemExt!();