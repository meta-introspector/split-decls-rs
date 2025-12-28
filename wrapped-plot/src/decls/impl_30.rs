macro_rules! deps {
    () => {
        Grid!();
        Configure!();
        Properties!();
        Default!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Configure < Grid > for Properties { type Properties = grid :: Properties ; # [doc = " Configures the gridlines"] fn configure < F > (& mut self , grid : Grid , configure : F) -> & mut Properties where F : FnOnce (& mut grid :: Properties) -> & mut grid :: Properties , { if self . grids . contains_key (grid) { configure (self . grids . get_mut (grid) . unwrap ()) ; } else { let mut properties = Default :: default () ; configure (& mut properties) ; self . grids . insert (grid , properties) ; } self } }
    };
}

impl_30!();