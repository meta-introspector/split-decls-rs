// Generated macro for BuildingConnectionSummary (struct)
macro_rules! Depcrate_client_async_clientBuildingConnectionSummary {
() => {
// Module: crate::client::async_client
// Provides: {"BuildingConnectionSummary"}
// Dependencies: {}
# [doc = " The [`Future`] used to build a [`ConnectionSummary`]."] # [doc = ""] # [doc = " At a high level, [`H3iDriver`] will interact with the UDP socket directly,"] # [doc = " sending and receiving data as necessary. As new data is received, it will"] # [doc = " send [`ConnectionRecord`]s to this struct, which uses these records to"] # [doc = " construct the [`ConnectionSummary`]."] # [must_use = "must await to get a ConnectionSummary"] pub struct BuildingConnectionSummary { rx : mpsc :: UnboundedReceiver < ConnectionRecord > , summary : Option < ConnectionSummary > , seen_all_close_trigger_frames : Option < oneshot :: Sender < () > > , }
};
}
