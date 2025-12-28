macro_rules! deps {
    () => {
        QSelf!();
        StructRest!();
        Walkable!();
        Path!();
        ExprField!();
    };
}

macro_rules! StructExpr {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct StructExpr { pub qself : Option < Box < QSelf > > , pub path : Path , pub fields : ThinVec < ExprField > , pub rest : StructRest , }
    };
}

StructExpr!();