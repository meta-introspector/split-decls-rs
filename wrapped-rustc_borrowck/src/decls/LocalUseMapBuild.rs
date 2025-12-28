macro_rules! deps {
    () => {
        LocalUseMap!();
    };
}

macro_rules! LocalUseMapBuild {
    () => {
        deps!();
        struct LocalUseMapBuild < 'me > { local_use_map : & 'me mut LocalUseMap , location_map : & 'me DenseLocationMap , locals_with_use_data : IndexVec < Local , bool > , }
    };
}

LocalUseMapBuild!();