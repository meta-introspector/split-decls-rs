// Generated macro for Events (struct)
macro_rules! DepcrateEvents {
() => {
// Module: crate
// Provides: {"Events"}
// Dependencies: {}
# [doc = " A container for I/O events."] pub struct Events { events : sys :: Events , # [doc = " This is intended to be used from &mut, thread locally, so we should make it !Sync"] # [doc = " for consistency with the rest of the API."] _not_sync : PhantomData < Cell < () > > , }
};
}
