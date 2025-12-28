macro_rules! deps {
    () => {
        TypedValueParser!();
    };
}

macro_rules! MapValueParser {
    () => {
        deps!();
        # [doc = " Adapt a `TypedValueParser` from one value to another"] # [doc = ""] # [doc = " See [`TypedValueParser::map`]"] # [derive (Clone , Debug)] pub struct MapValueParser < P , F > { parser : P , func : F , }
    };
}

MapValueParser!();