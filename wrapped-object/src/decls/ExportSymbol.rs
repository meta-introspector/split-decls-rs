macro_rules! deps {
    () => {
        ExportData!();
    };
}

macro_rules! ExportSymbol {
    () => {
        deps!();
        # [doc = " Exported symbol information."] # [derive (Debug)] pub struct ExportSymbol < 'data > { name : Box < [u8] > , flags : u8 , data : ExportData < 'data > , }
    };
}

ExportSymbol!();