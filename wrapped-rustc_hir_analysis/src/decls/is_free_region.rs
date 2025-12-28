macro_rules! is_free_region {
    () => {
        fn is_free_region (region : Region < '_ >) -> bool { match region . kind () { ty :: ReEarlyParam (_) => true , ty :: ReStatic => false , ty :: ReBound (..) => false , ty :: ReError (_) => false , ty :: ReErased | ty :: ReVar (..) | ty :: RePlaceholder (..) | ty :: ReLateParam (..) => { bug ! ("unexpected region in outlives inference: {:?}" , region) ; } } }
    };
}

is_free_region!()