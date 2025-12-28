macro_rules! intersect {
    () => {
        fn intersect (dominators : & [usize] , mut finger1 : usize , mut finger2 : usize) -> usize { loop { match finger1 . cmp (& finger2) { Ordering :: Less => finger1 = dominators [finger1] , Ordering :: Greater => finger2 = dominators [finger2] , Ordering :: Equal => return finger1 , } } }
    };
}

intersect!();