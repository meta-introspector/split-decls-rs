macro_rules! deps {
    () => {
        Float!();
        Resamples!();
        Tuple!();
        Sample!();
        Data!();
        Distributions!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl < 'a , X , Y > Data < 'a , X , Y > where X : Float , Y : Float , { # [doc = " Creates a new data set from two existing slices"] pub fn new (xs : & 'a [X] , ys : & 'a [Y]) -> Data < 'a , X , Y > { assert ! (xs . len () == ys . len () && xs . len () > 1 && xs . iter () . all (| x | ! x . is_nan ()) && ys . iter () . all (| y | ! y . is_nan ())) ; Data (xs , ys) } # [doc = " Returns the bootstrap distributions of the parameters estimated by the `statistic`"] # [doc = ""] # [doc = " - Multi-threaded"] # [doc = " - Time: `O(nresamples)`"] # [doc = " - Memory: `O(nresamples)`"] pub fn bootstrap < T , S > (& self , nresamples : usize , statistic : S) -> T :: Distributions where S : Fn (Data < X , Y >) -> T + Sync , T : Tuple + Send , T :: Distributions : Send , T :: Builder : Send , { # [cfg (feature = "rayon")] { (0 .. nresamples) . into_par_iter () . map_init (| | Resamples :: new (* self) , | resamples , _ | statistic (resamples . next ()) ,) . fold (| | T :: Builder :: new (0) , | mut sub_distributions , sample | { sub_distributions . push (sample) ; sub_distributions } ,) . reduce (| | T :: Builder :: new (0) , | mut a , mut b | { a . extend (& mut b) ; a } ,) . complete () } # [cfg (not (feature = "rayon"))] { let mut resamples = Resamples :: new (* self) ; (0 .. nresamples) . map (| _ | statistic (resamples . next ())) . fold (T :: Builder :: new (0) , | mut sub_distributions , sample | { sub_distributions . push (sample) ; sub_distributions }) . complete () } } # [doc = " Returns a view into the `X` data"] pub fn x (& self) -> & 'a Sample < X > { Sample :: new (self . 0) } # [doc = " Returns a view into the `Y` data"] pub fn y (& self) -> & 'a Sample < Y > { Sample :: new (self . 1) } }
    };
}

impl_307!();