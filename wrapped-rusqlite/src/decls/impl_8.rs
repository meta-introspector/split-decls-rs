macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Self :: SqliteFailure (ref err , _) => Some (err) , Self :: Utf8Error (ref err) => Some (err) , Self :: NulError (ref err) => Some (err) , Self :: IntegralValueOutOfRange (..) | Self :: SqliteSingleThreadedMode | Self :: InvalidParameterName (_) | Self :: ExecuteReturnedResults | Self :: QueryReturnedNoRows | Self :: QueryReturnedMoreThanOneRow | Self :: InvalidColumnIndex (_) | Self :: InvalidColumnName (_) | Self :: InvalidColumnType (..) | Self :: InvalidPath (_) | Self :: InvalidParameterCount (..) | Self :: StatementChangedRows (_) | Self :: InvalidQuery | Self :: MultipleStatement => None , # [cfg (feature = "functions")] Self :: InvalidFunctionParameterType (..) => None , # [cfg (feature = "vtab")] Self :: InvalidFilterParameterType (..) => None , # [cfg (feature = "functions")] Self :: UserFunctionError (ref err) => Some (& * * err) , Self :: FromSqlConversionFailure (_ , _ , ref err) | Self :: ToSqlConversionFailure (ref err) => Some (& * * err) , # [cfg (feature = "vtab")] Self :: ModuleError (_) => None , Self :: UnwindingPanic => None , # [cfg (feature = "functions")] Self :: GetAuxWrongType => None , # [cfg (feature = "blob")] Self :: BlobSizeError => None , # [cfg (feature = "modern_sqlite")] Self :: SqlInputError { ref error , .. } => Some (error) , # [cfg (feature = "loadable_extension")] Self :: InitError (ref err) => Some (err) , # [cfg (feature = "modern_sqlite")] Self :: InvalidDatabaseIndex (_) => None , } } }
    };
}

impl_8!()