// Generated macro for SpawnResultWithReturnChannelAndWorkers (type)
macro_rules! Depcrate_hours_coreSpawnResultWithReturnChannelAndWorkers {
() => {
// Module: crate::hours::core
// Provides: {"SpawnResultWithReturnChannelAndWorkers"}
// Dependencies: {}
type SpawnResultWithReturnChannelAndWorkers < 'scope > = (crossbeam_channel :: Sender < Vec < (CommitIdx , Option < gix :: hash :: ObjectId > , gix :: hash :: ObjectId) > > , Vec < std :: thread :: ScopedJoinHandle < 'scope , anyhow :: Result < Vec < (CommitIdx , FileStats , LineStats) > > > > ,) ;
};
}
