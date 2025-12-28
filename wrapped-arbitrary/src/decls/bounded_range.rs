macro_rules! bounded_range {
    () => {
        pub (crate) fn bounded_range < CB , I , R > (bounds : (I , I) , cb : CB) -> R where CB : Fn ((I , I)) -> R , I : PartialOrd , R : RangeBounds < I > , { let (mut start , mut end) = bounds ; if start > end { mem :: swap (& mut start , & mut end) ; } cb ((start , end)) }
    };
}

bounded_range!();