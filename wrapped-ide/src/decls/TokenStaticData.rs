macro_rules! deps {
    () => {
        MonikerResult!();
        ReferenceData!();
        SymbolInformationKind!();
        HoverResult!();
    };
}

macro_rules! TokenStaticData {
    () => {
        deps!();
        # [derive (Debug)] pub struct TokenStaticData { pub documentation : Option < Documentation > , pub hover : Option < HoverResult > , pub definition : Option < FileRange > , pub references : Vec < ReferenceData > , pub moniker : Option < MonikerResult > , pub display_name : Option < String > , pub signature : Option < String > , pub kind : SymbolInformationKind , }
    };
}

TokenStaticData!();