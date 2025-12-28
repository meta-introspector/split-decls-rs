macro_rules! into_group_map_by {
    () => {
        pub fn into_group_map_by < I , K , V , F > (iter : I , mut f : F) -> HashMap < K , Vec < V > > where I : Iterator < Item = V > , K : Hash + Eq , F : FnMut (& V) -> K , { into_group_map (iter . map (| v | (f (& v) , v))) }
    };
}

into_group_map_by!()