macro_rules! deps {
    () => {
        U16!();
    };
}

macro_rules! ImageImportByName {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageImportByName { pub hint : U16 < LE > , }
    };
}

ImageImportByName!();