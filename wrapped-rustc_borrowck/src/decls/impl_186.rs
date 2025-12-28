macro_rules! deps {
    () => {
        RegionTracker!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl scc :: Annotation for RegionTracker { fn merge_scc (self , other : Self) -> Self { trace ! ("{:?} << {:?}" , self . representative , other . representative) ; Self { representative : self . representative . min (other . representative) , max_nameable_universe : self . max_nameable_universe . min (other . max_nameable_universe) , reachable_placeholders : self . reachable_placeholders . merge (other . reachable_placeholders) , } } fn merge_reached (self , other : Self) -> Self { Self { max_nameable_universe : self . max_nameable_universe . min (other . max_nameable_universe) , reachable_placeholders : self . reachable_placeholders . merge (other . reachable_placeholders) , representative : self . representative , } } }
    };
}

impl_186!()