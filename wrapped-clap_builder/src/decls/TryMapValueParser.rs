macro_rules! deps {
    () => {
        TypedValueParser!();
    };
}

macro_rules! TryMapValueParser {
    () => {
        deps!();
        # [doc = " Adapt a `TypedValueParser` from one value to another"] # [doc = ""] # [doc = " See [`TypedValueParser::try_map`]"] # [derive (Clone , Debug)] pub struct TryMapValueParser < P , F > { parser : P , func : F , }
    };
}

TryMapValueParser!()