macro_rules! deps {
    () => {
        Reader!();
    };
}

macro_rules! LineInstructions {
    () => {
        deps!();
        # [doc = " An iterator yielding parsed instructions."] # [doc = ""] # [doc = " See"] # [doc = " [`LineProgramHeader::instructions`](./struct.LineProgramHeader.html#method.instructions)"] # [doc = " for more details."] # [derive (Clone , Debug)] pub struct LineInstructions < R : Reader > { input : R , }
    };
}

LineInstructions!()