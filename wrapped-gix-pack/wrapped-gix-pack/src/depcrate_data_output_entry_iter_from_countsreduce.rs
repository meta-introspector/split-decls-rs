// Generated macro for reduce (module)
macro_rules! Depcrate_data_output_entry_iter_from_countsreduce {
() => {
// Module: crate::data::output::entry::iter_from_counts
// Provides: {"reduce"}
// Dependencies: {}
mod reduce { use std :: marker :: PhantomData ; use gix_features :: { parallel , parallel :: SequenceId } ; use super :: Outcome ; use crate :: data :: output ; pub struct Statistics < E > { total : Outcome , _err : PhantomData < E > , } impl < E > Default for Statistics < E > { fn default () -> Self { Statistics { total : Default :: default () , _err : PhantomData , } } } impl < Error > parallel :: Reduce for Statistics < Error > { type Input = Result < (SequenceId , Vec < output :: Entry > , Outcome) , Error > ; type FeedProduce = (SequenceId , Vec < output :: Entry >) ; type Output = Outcome ; type Error = Error ; fn feed (& mut self , item : Self :: Input) -> Result < Self :: FeedProduce , Self :: Error > { item . map (| (cid , entries , stats) | { self . total . aggregate (stats) ; (cid , entries) }) } fn finalize (self) -> Result < Self :: Output , Self :: Error > { Ok (self . total) } } }
};
}
