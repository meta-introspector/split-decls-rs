// Generated macro for InterruptingWriter (struct)
macro_rules! Depcrate_write_encoder_testsInterruptingWriter {
() => {
// Module: crate::write::encoder_tests
// Provides: {"InterruptingWriter"}
// Dependencies: {}
# [doc = " A `Write` implementation that returns Interrupted some fraction of the time, randomly."] struct InterruptingWriter < 'a , W : 'a + Write , R : 'a + Rng > { w : & 'a mut W , rng : & 'a mut R , # [doc = " In [0, 1]. If a random number in [0, 1] is  `<= threshold`, `Write` methods will return"] # [doc = " an `Interrupted` error"] fraction : f64 , }
};
}
