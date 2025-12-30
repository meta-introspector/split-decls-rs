// Generated macro for impl_19 (impl)
macro_rules! Depcrate_eventsimpl_19 {
() => {
// Module: crate::events
// Provides: {"impl_19"}
// Dependencies: {}
impl EventKind { const BITFLAG_ENUM_MAP : & [(EventMask , EventKind)] = & [(EventMask :: ACCESS , EventKind :: Access) , (EventMask :: ATTRIB , EventKind :: Attrib) , (EventMask :: CLOSE_WRITE , EventKind :: CloseWrite) , (EventMask :: CLOSE_NOWRITE , EventKind :: CloseNowrite) , (EventMask :: CREATE , EventKind :: Create) , (EventMask :: DELETE , EventKind :: Delete) , (EventMask :: DELETE_SELF , EventKind :: DeleteSelf) , (EventMask :: MODIFY , EventKind :: Modify) , (EventMask :: MOVE_SELF , EventKind :: MoveSelf) , (EventMask :: MOVED_FROM , EventKind :: MovedFrom) , (EventMask :: MOVED_TO , EventKind :: MovedTo) , (EventMask :: OPEN , EventKind :: Open) ,] ; # [doc = " Parse the auxiliary flags from a raw event mask"] pub fn from_raw_event_mask (mask : EventMask) -> Result < Option < Self > , EventMaskParseError > { let mut kinds = Self :: BITFLAG_ENUM_MAP . iter () . filter_map (| bf_map | { if mask . contains (bf_map . 0) { Some (bf_map . 1) } else { None } }) ; let kind = kinds . next () ; if kinds . next () . is_some () { return Err (EventMaskParseError :: TooManyBitsSet (mask)) ; } Ok (kind) } }
};
}
