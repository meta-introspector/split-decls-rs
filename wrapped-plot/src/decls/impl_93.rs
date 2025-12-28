macro_rules! deps {
    () => {
        Axis!();
        Script!();
        Properties!();
        Grid!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Script for (Axis , Grid , & Properties) { fn script (& self) -> String { let & (axis , grid , properties) = self ; let axis = axis . display () ; let grid = grid . display () ; if properties . hidden { String :: new () } else { format ! ("set grid {}{}tics\n" , grid , axis) } } }
    };
}

impl_93!();