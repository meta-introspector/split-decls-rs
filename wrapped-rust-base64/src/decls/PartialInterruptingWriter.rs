macro_rules! PartialInterruptingWriter {
    () => {
        # [doc = " A `Write` implementation that sometimes will only write part of its input."] struct PartialInterruptingWriter < 'a , W : 'a + Write , R : 'a + Rng > { w : & 'a mut W , rng : & 'a mut R , # [doc = " In [0, 1]. If a random number in [0, 1] is  `<= threshold`, `write()` will write all its"] # [doc = " input. Otherwise, it will write a random substring"] full_input_fraction : f64 , no_interrupt_fraction : f64 , }
    };
}

PartialInterruptingWriter!();