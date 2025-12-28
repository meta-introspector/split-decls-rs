macro_rules! deps {
    () => {
        Ty!();
        Generics!();
        TraitImplHeader!();
        AssocItem!();
    };
}

macro_rules! Impl {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug)] pub struct Impl { pub generics : Generics , pub of_trait : Option < Box < TraitImplHeader > > , pub self_ty : Box < Ty > , pub items : ThinVec < Box < AssocItem > > , }
    };
}

Impl!()