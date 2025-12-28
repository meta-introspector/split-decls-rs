macro_rules! TraitOrTraitImpl {
    () => {
        enum TraitOrTraitImpl { Trait { span : Span , constness : Const } , TraitImpl { constness : Const , polarity : ImplPolarity , trait_ref_span : Span } , }
    };
}

TraitOrTraitImpl!();