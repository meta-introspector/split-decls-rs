macro_rules! deps {
    () => {
        CaptureLocations!();
    };
}

macro_rules! Locations {
    () => {
        deps!();
        # [doc = " A type alias for `CaptureLocations` for backwards compatibility."] # [doc = ""] # [doc = " Previously, we exported `CaptureLocations` as `Locations` in an"] # [doc = " undocumented API. To prevent breaking that code (e.g., in `regex-capi`),"] # [doc = " we continue re-exporting the same undocumented API."] # [doc (hidden)] pub type Locations = CaptureLocations ;
    };
}

Locations!()