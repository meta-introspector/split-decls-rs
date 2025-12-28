macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! ImportThunkList {
    () => {
        deps!();
        # [doc = " A list of import thunks."] # [doc = ""] # [doc = " These may be in the import lookup table, or the import address table."] # [derive (Debug , Clone)] pub struct ImportThunkList < 'data > { data : Bytes < 'data > , }
    };
}

ImportThunkList!()