// Generated macro for StatefulDecoderHelper (struct)
macro_rules! Depcrate_utilStatefulDecoderHelper {
() => {
// Module: crate::util
// Provides: {"StatefulDecoderHelper"}
// Dependencies: {}
# [doc = " A helper struct for the stateful decoder DSL."] pub struct StatefulDecoderHelper < 'a , St , Data : 'a > { # [doc = " The current buffer."] pub buf : & 'a [u8] , # [doc = " The current index to the buffer."] pub pos : usize , # [doc = " The output buffer."] pub output : & 'a mut (types :: StringWriter + 'a) , # [doc = " The last codec error. The caller will later collect this."] pub err : Option < types :: CodecError > , # [doc = " The additional data attached for the use from transition functions."] pub data : & 'a Data , # [doc = " A marker for the phantom type parameter `St`."] _marker : PhantomData < St > , }
};
}
