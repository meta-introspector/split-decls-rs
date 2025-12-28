macro_rules! deps {
    () => {
        RangedI64ValueParser!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < T : TryFrom < i64 > + Clone + Send + Sync > RangedI64ValueParser < T > { # [doc = " Select full range of `i64`"] pub fn new () -> Self { Self :: from (..) } # [doc = " Narrow the supported range"] pub fn range < B : RangeBounds < i64 > > (mut self , range : B) -> Self { let start = match range . start_bound () { l @ std :: ops :: Bound :: Included (i) => { debug_assert ! (self . bounds . contains (i) , "{} must be in {:?}" , i , self . bounds) ; l . cloned () } l @ std :: ops :: Bound :: Excluded (i) => { debug_assert ! (self . bounds . contains (& i . saturating_add (1)) , "{} must be in {:?}" , i , self . bounds) ; l . cloned () } std :: ops :: Bound :: Unbounded => self . bounds . start_bound () . cloned () , } ; let end = match range . end_bound () { l @ std :: ops :: Bound :: Included (i) => { debug_assert ! (self . bounds . contains (i) , "{} must be in {:?}" , i , self . bounds) ; l . cloned () } l @ std :: ops :: Bound :: Excluded (i) => { debug_assert ! (self . bounds . contains (& i . saturating_sub (1)) , "{} must be in {:?}" , i , self . bounds) ; l . cloned () } std :: ops :: Bound :: Unbounded => self . bounds . end_bound () . cloned () , } ; self . bounds = (start , end) ; self } fn format_bounds (& self) -> String { let mut result = match self . bounds . 0 { std :: ops :: Bound :: Included (i) => i . to_string () , std :: ops :: Bound :: Excluded (i) => i . saturating_add (1) . to_string () , std :: ops :: Bound :: Unbounded => i64 :: MIN . to_string () , } ; result . push_str ("..") ; match self . bounds . 1 { std :: ops :: Bound :: Included (i) => { result . push ('=') ; result . push_str (& i . to_string ()) ; } std :: ops :: Bound :: Excluded (i) => { result . push_str (& i . to_string ()) ; } std :: ops :: Bound :: Unbounded => { result . push_str (& i64 :: MAX . to_string ()) ; } } result } }
    };
}

impl_290!()