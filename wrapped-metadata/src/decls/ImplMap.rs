macro_rules! deps {
    () => {
        ModuleRef!();
    };
}

macro_rules! ImplMap {
    () => {
        deps!();
        pub struct ImplMap { pub MappingFlags : PInvokeAttributes , pub MemberForwarded : MemberForwarded , pub ImportName : id :: StringId , pub ImportScope : id :: ModuleRef , }
    };
}

ImplMap!()