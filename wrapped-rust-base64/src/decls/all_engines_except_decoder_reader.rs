macro_rules! deps {
    () => {
        GeneralPurposeWrapper!();
        NaiveWrapper!();
        EngineWrapper!();
        DecoderReader!();
    };
}

macro_rules! all_engines_except_decoder_reader {
    () => {
        deps!();
        # [doc = " Some decode tests don't make sense for use with `DecoderReader` as they are difficult to"] # [doc = " reason about or otherwise inapplicable given how DecoderReader slice up its input along"] # [doc = " chunk boundaries."] # [template] # [rstest (engine_wrapper , case :: general_purpose (GeneralPurposeWrapper { }) , case :: naive (NaiveWrapper { }) ,)] fn all_engines_except_decoder_reader < E : EngineWrapper > (engine_wrapper : E) { }
    };
}

all_engines_except_decoder_reader!();