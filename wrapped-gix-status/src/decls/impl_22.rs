macro_rules! deps {
    () => {
        SubmoduleStatus!();
        VisitEntry!();
        Error!();
        ReduceChange!();
        StatusResult!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'index , T , U , C : VisitEntry < 'index , ContentChange = T , SubmoduleStatus = U > > Reduce for ReduceChange < '_ , 'index , C > { type Input = Vec < StatusResult < 'index , T , U > > ; type FeedProduce = () ; type Output = () ; type Error = Error ; fn feed (& mut self , items : Self :: Input) -> Result < Self :: FeedProduce , Self :: Error > { for item in items { let (entry , entry_index , path , status) = item ? ; self . collector . visit_entry (self . entries , entry , entry_index , path , status) ; } Ok (()) } fn finalize (self) -> Result < Self :: Output , Self :: Error > { Ok (()) } }
    };
}

impl_22!()