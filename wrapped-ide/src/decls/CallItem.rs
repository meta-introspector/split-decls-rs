macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! CallItem {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct CallItem { pub target : NavigationTarget , pub ranges : Vec < FileRange > , }
    };
}

CallItem!()