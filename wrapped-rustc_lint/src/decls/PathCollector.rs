macro_rules! PathCollector {
    () => {
        # [doc = " Simple hir::Path collector"] struct PathCollector < 'tcx > { paths : Vec < Path < 'tcx > > , }
    };
}

PathCollector!();