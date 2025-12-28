macro_rules! deps {
    () => {
        TraitRef!();
        Safety!();
        Defaultness!();
        Const!();
        ImplPolarity!();
    };
}

macro_rules! TraitImplHeader {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug)] pub struct TraitImplHeader { pub defaultness : Defaultness , pub safety : Safety , pub constness : Const , pub polarity : ImplPolarity , pub trait_ref : TraitRef , }
    };
}

TraitImplHeader!();