macro_rules! deps {
    () => {
        LinearMap!();
        Vec!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < K , V , const N : usize > LinearMap < K , V , N > { # [doc = " Creates an empty `LinearMap`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::LinearMap;"] # [doc = ""] # [doc = " // allocate the map on the stack"] # [doc = " let mut map: LinearMap<&str, isize, 8> = LinearMap::new();"] # [doc = ""] # [doc = " // allocate the map in a static variable"] # [doc = " static mut MAP: LinearMap<&str, isize, 8> = LinearMap::new();"] # [doc = " ```"] pub const fn new () -> Self { Self { buffer : Vec :: new () } } }
    };
}

impl_169!()