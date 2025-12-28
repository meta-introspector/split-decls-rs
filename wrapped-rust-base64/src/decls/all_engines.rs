macro_rules! deps {
    () => {
        GeneralPurposeWrapper!();
        NaiveWrapper!();
        EngineWrapper!();
        DecoderReaderEngineWrapper!();
    };
}

macro_rules! all_engines {
    () => {
        deps!();
        # [template] # [rstest (engine_wrapper , case :: general_purpose (GeneralPurposeWrapper { }) , case :: naive (NaiveWrapper { }) , case :: decoder_reader (DecoderReaderEngineWrapper { }) ,)] fn all_engines < E : EngineWrapper > (engine_wrapper : E) { }
    };
}

all_engines!();