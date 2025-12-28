macro_rules! Inspect {
    () => {
        # [doc = " An iterator which passes each element to a closure before returning it."] # [derive (Clone , Debug)] pub struct Inspect < I , F > { it : I , f : F , }
    };
}

Inspect!()