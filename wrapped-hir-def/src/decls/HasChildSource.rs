macro_rules! deps {
    () => {
        DefDatabase!();
    };
}

macro_rules! HasChildSource {
    () => {
        deps!();
        pub trait HasChildSource < ChildId > { type Value ; fn child_source (& self , db : & dyn DefDatabase) -> InFile < ArenaMap < ChildId , Self :: Value > > ; }
    };
}

HasChildSource!()