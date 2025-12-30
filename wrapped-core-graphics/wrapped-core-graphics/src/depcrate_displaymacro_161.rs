// Generated macro for macro_161 (macro)
macro_rules! Depcrate_displaymacro_161 {
() => {
// Module: crate::display
// Provides: {"macro_161"}
// Dependencies: {}
bitflags ! { # [doc = " The configuration parameters that are passed to a display reconfiguration callback function."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct CGDisplayChangeSummaryFlags : u32 { # [doc = " The display configuration is about to change."] const kCGDisplayBeginConfigurationFlag = 1 ; # [doc = " The location of the upper-left corner of the display in the global display coordinate space has changed."] const kCGDisplayMovedFlag = 1 << 1 ; # [doc = " The display is now the main display."] const kCGDisplaySetMainFlag = 1 << 2 ; # [doc = " The display mode has changed."] const kCGDisplaySetModeFlag = 1 << 3 ; # [doc = " The display has been added to the active display list."] const kCGDisplayAddFlag = 1 << 4 ; # [doc = " The display has been removed from the active display list."] const kCGDisplayRemoveFlag = 1 << 5 ; # [doc = " The display has been enabled."] const kCGDisplayEnabledFlag = 1 << 8 ; # [doc = " The display has been disabled."] const kCGDisplayDisabledFlag = 1 << 9 ; # [doc = " The display is now mirroring another display."] const kCGDisplayMirrorFlag = 1 << 10 ; # [doc = " The display is no longer mirroring another display."] const kCGDisplayUnMirrorFlag = 1 << 11 ; # [doc = " The shape of the desktop (the union of display areas) has changed."] const kCGDisplayDesktopShapeChangedFlag = 1 << 12 ; const _ = ! 0 ; } }
};
}
