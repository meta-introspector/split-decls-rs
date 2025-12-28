macro_rules! deps {
    () => {
        Place!();
    };
}

macro_rules! PlaceTy {
    () => {
        deps!();
        # [doc = " An evaluated place, together with its type."] # [doc = ""] # [doc = " This may reference a stack frame by its index, so `PlaceTy` should generally not be kept around"] # [doc = " for longer than a single operation. Popping and then pushing a stack frame can make `PlaceTy`"] # [doc = " point to the wrong destination. If the interpreter has multiple stacks, stack switching will"] # [doc = " also invalidate a `PlaceTy`."] # [derive (Clone)] pub struct PlaceTy < 'tcx , Prov : Provenance = CtfeProvenance > { place : Place < Prov > , pub layout : TyAndLayout < 'tcx > , }
    };
}

PlaceTy!();