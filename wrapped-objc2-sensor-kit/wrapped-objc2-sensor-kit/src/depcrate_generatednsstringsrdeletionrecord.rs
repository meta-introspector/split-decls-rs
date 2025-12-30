// Generated macro for NSStringSRDeletionRecord (trait)
macro_rules! Depcrate_generatedNSStringSRDeletionRecord {
() => {
// Module: crate::generated
// Provides: {"NSStringSRDeletionRecord"}
// Dependencies: {}
# [doc = " Category \"SRDeletionRecord\" on [`NSString`]."] # [doc (alias = "SRDeletionRecord")] pub unsafe trait NSStringSRDeletionRecord : ClassType + Sized + private_NSStringSRDeletionRecord :: Sealed { extern_methods ! (# [doc = " Returns a sensor stream that contains deletion records of the sensor"] # [doc = ""] # [doc = ""] # [doc = " This sensor stream should only be used for fetching. All other"] # [doc = " operations will be ignored. Deletion records share the recording and authorization"] # [doc = " state with their parent sensor."] # [doc = ""] # [doc = ""] # [doc = " Returns: May return nil if there is no deletion record available for this sensor"] # [unsafe (method (sr_sensorForDeletionRecordsFromSensor))] # [unsafe (method_family = none)] unsafe fn sr_sensorForDeletionRecordsFromSensor (& self) -> Option < Retained < SRSensor >>;) ; }
};
}
