macro_rules! deps {
    () => {
        DecoderReader!();
        Engine!();
    };
}

macro_rules! DecoderReaderEngine {
    () => {
        deps!();
        # [doc = " A pseudo-Engine that routes all decoding through [DecoderReader]"] struct DecoderReaderEngine < E : Engine > { engine : E , }
    };
}

DecoderReaderEngine!();