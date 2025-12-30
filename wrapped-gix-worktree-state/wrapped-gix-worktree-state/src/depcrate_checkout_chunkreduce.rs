// Generated macro for reduce (module)
macro_rules! Depcrate_checkout_chunkreduce {
() => {
// Module: crate::checkout::chunk
// Provides: {"reduce"}
// Dependencies: {}
mod reduce { use crate :: checkout ; pub struct Reduce < 'entry > { pub aggregate : super :: Outcome < 'entry > , } impl < 'entry > gix_features :: parallel :: Reduce for Reduce < 'entry > { type Input = Result < super :: Outcome < 'entry > , checkout :: Error > ; type FeedProduce = () ; type Output = super :: Outcome < 'entry > ; type Error = checkout :: Error ; fn feed (& mut self , item : Self :: Input) -> Result < Self :: FeedProduce , Self :: Error > { let item = item ? ; let super :: Outcome { bytes_written , files , delayed_symlinks , errors , collisions , delayed_paths_unknown , delayed_paths_unprocessed , } = item ; self . aggregate . bytes_written += bytes_written ; self . aggregate . files += files ; self . aggregate . delayed_symlinks . extend (delayed_symlinks) ; self . aggregate . errors . extend (errors) ; self . aggregate . collisions . extend (collisions) ; self . aggregate . delayed_paths_unknown . extend (delayed_paths_unknown) ; self . aggregate . delayed_paths_unprocessed . extend (delayed_paths_unprocessed) ; Ok (()) } fn finalize (self) -> Result < Self :: Output , Self :: Error > { Ok (self . aggregate) } } }
};
}
