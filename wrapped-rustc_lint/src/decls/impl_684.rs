macro_rules! deps {
    () => {
        PathCollector!();
    };
}

macro_rules! impl_684 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for PathCollector < 'tcx > { fn visit_path (& mut self , path : & Path < 'tcx > , _id : HirId) { self . paths . push (path . clone ()) ; intravisit :: walk_path (self , path) } }
    };
}

impl_684!()