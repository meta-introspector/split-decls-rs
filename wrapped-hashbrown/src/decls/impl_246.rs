macro_rules! deps {
    () => {
        DefaultHashBuilder!();
        HashMap!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        # [cfg (feature = "default-hasher")] impl < K , V , A , const N : usize > From < [(K , V) ; N] > for HashMap < K , V , DefaultHashBuilder , A > where K : Eq + Hash , A : Default + Allocator , { # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map1 = HashMap::from([(1, 2), (3, 4)]);"] # [doc = " let map2: HashMap<_, _> = [(1, 2), (3, 4)].into();"] # [doc = " assert_eq!(map1, map2);"] # [doc = " ```"] fn from (arr : [(K , V) ; N]) -> Self { arr . into_iter () . collect () } }
    };
}

impl_246!();