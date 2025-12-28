macro_rules! deps {
    () => {
        Engine!();
    };
}

macro_rules! ChunkedEncoder {
    () => {
        deps!();
        # [doc = " A base64 encoder that emits encoded bytes in chunks without heap allocation."] pub struct ChunkedEncoder < 'e , E : Engine + ? Sized > { engine : & 'e E , }
    };
}

ChunkedEncoder!();