macro_rules! deps {
    () => {
        LifetimeInfoMap!();
    };
}

macro_rules! LifetimeInfoCollector {
    () => {
        deps!();
        struct LifetimeInfoCollector < 'a , 'tcx > { type_span : Span , referenced_type_span : Option < Span > , map : & 'a mut LifetimeInfoMap < 'tcx > , }
    };
}

LifetimeInfoCollector!();