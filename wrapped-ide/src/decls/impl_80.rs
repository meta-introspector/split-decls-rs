macro_rules! deps {
    () => {
        CallLocations!();
        CallItem!();
        NavigationTarget!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl CallLocations { fn add (& mut self , target : NavigationTarget , range : FileRange) { self . funcs . entry (target) . or_default () . push (range) ; } fn into_items (self) -> Vec < CallItem > { self . funcs . into_iter () . map (| (target , ranges) | CallItem { target , ranges }) . collect () } }
    };
}

impl_80!()