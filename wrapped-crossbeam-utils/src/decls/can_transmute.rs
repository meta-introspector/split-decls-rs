macro_rules! can_transmute {
    () => {
        # [doc = " Returns `true` if values of type `A` can be transmuted into values of type `B`."] const fn can_transmute < A , B > () -> bool { (mem :: size_of :: < A > () == mem :: size_of :: < B > ()) & (mem :: align_of :: < A > () >= mem :: align_of :: < B > ()) }
    };
}

can_transmute!()