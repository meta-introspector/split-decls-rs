macro_rules! deps {
    () => {
        Constness!();
        TraitRef!();
        Safety!();
        Defaultness!();
    };
}

macro_rules! TraitImplHeader {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct TraitImplHeader < 'hir > { pub constness : Constness , pub safety : Safety , pub polarity : ImplPolarity , pub defaultness : Defaultness , pub defaultness_span : Option < Span > , pub trait_ref : TraitRef < 'hir > , }
    };
}

TraitImplHeader!()