macro_rules! deps {
    () => {
        UndoLog!();
        RegionConstraintStorage!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'tcx > Rollback < UndoLog < 'tcx > > for RegionConstraintStorage < 'tcx > { fn reverse (& mut self , undo : UndoLog < 'tcx >) { match undo { AddVar (vid) => { self . var_infos . pop () . unwrap () ; assert_eq ! (self . var_infos . len () , vid . index ()) ; } AddConstraint (index) => { self . data . constraints . pop () . unwrap () ; assert_eq ! (self . data . constraints . len () , index) ; } AddVerify (index) => { self . data . verifys . pop () ; assert_eq ! (self . data . verifys . len () , index) ; } AddCombination (Glb , ref regions) => { self . glbs . remove (regions) ; } AddCombination (Lub , ref regions) => { self . lubs . remove (regions) ; } } } }
    };
}

impl_147!()