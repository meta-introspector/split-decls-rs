macro_rules! deps {
    () => {
        GenericBounds!();
        Walkable!();
        BoundKind!();
        Safety!();
        Const!();
        Generics!();
        IsAuto!();
        AssocCtxt!();
        AssocItem!();
    };
}

macro_rules! Trait {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Trait { pub constness : Const , pub safety : Safety , pub is_auto : IsAuto , pub ident : Ident , pub generics : Generics , # [visitable (extra = BoundKind :: SuperTraits)] pub bounds : GenericBounds , # [visitable (extra = AssocCtxt :: Trait)] pub items : ThinVec < Box < AssocItem > > , }
    };
}

Trait!();